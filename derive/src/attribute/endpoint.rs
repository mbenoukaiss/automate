use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::{parse_macro_input, Ident, AttributeArgs, ItemFn, Error};
use quote::quote;
use darling::FromMeta;
use crate::utils;

/// Default user agent for automate bots
const USER_AGENT: &str = concat!("DiscordBot (https://github.com/mbenoukaiss/automate, ", env!("CARGO_PKG_VERSION"), ")");

/// Generates the code that should build
/// the route URL based on the parameters.

/// Generate the code that should send the request
/// to Discord's servers.
fn generate_request(item: &ItemFn, args: Args, major_parameter: TokenStream2) -> Result<TokenStream2, TokenStream> {
    let fn_name = &item.sig.ident;

    let method = args.method()?;
    let uri = args.route();
    let content_type = args.content_type();
    let body = args.body();
    let status = args.status;

    // hyper does not set content-length to 0 when the body is
    // empty and method is POST, PUT or PATCH, but discord
    // requires a content-length
    let zero_content_length = match method.to_string().as_str() {
        "POST" | "PUT" | "PATCH" if !args.multipart => Some(quote! {
            match ::hyper::body::HttpBody::size_hint(&#body).exact() {
                Some(0) | None => request = request.header("Content-Length", 0),
                _ => ()
            };
        }),
        _ => None
    };

    let return_value = if args.empty {
        quote!(Ok(()))
    } else if cfg!(feature = "trace-endpoints") {
        quote! {{
            let body = hyper::body::aggregate(response).await?;

            let mut body_string = String::new();
            let mut reader = bytes::buf::Buf::reader(body);
            std::io::Read::read_to_string(&mut reader, &mut body_string)?;

            log::trace!("Endpoint `{}` responded: {}", stringify!(#fn_name), body_string);

            Ok(::serde_json::from_str(&body_string)?)
        }}
    } else {
        quote! {{
            let body = ::hyper::body::aggregate(response).await?;
            Ok(::serde_json::from_reader(::bytes::buf::Buf::reader(body))?)
        }}
    };

    Ok(quote! {
        use tokio::sync::RwLock;
        use rate_limit::{Key, Bucket, BUCKETS};

        ::lazy_static::lazy_static! {
            static ref BUCKET_ID: RwLock<Option<String>> = RwLock::default();
        }

        if let Some(bucket_id) = BUCKET_ID.read().await.as_ref() {
            if let Some(bucket) = BUCKETS.lock().await.get(&Key::lookup(&self.token, &bucket_id, #major_parameter)) {
                trace!("Endpoint {}#{} allows for {} more calls (limit {})", stringify!(#fn_name), bucket.id, bucket.remaining, bucket.limit);

                if bucket.remaining == 0 && ::chrono::Utc::now().naive_utc() < bucket.reset {
                    return Error::rate_limited(stringify!(#fn_name), bucket.reset, true);
                }
            }
        }

        let uri = #uri;

        let mut request = ::hyper::Request::builder()
            .uri(uri.clone())
            .method(::hyper::Method::#method)
            .header("Content-Type", #content_type)
            .header("Authorization", &self.token)
            .header("User-Agent", #USER_AGENT)
            .header("X-RateLimit-Precision", "millisecond");

        #zero_content_length

        let response = self.client.request(request.body(#body).unwrap()).await?;

        if let Some(bucket) = Bucket::new(response.headers())? {
            BUCKET_ID.write().await.replace(bucket.id.clone());
            BUCKETS.lock().await.insert(Key::insert(self.token.clone(), bucket.id.clone(), #major_parameter), bucket);
        }

        let code = response.status().as_u16();

        match code {
            #status => #return_value,
            400 => Error::http(format!("Bad request (endpoint `̀{}`)", stringify!(#fn_name))),
            401 => Error::invalid_token(stringify!(#fn_name), &self.token),
            403 => Error::no_permission(stringify!(#fn_name), &self.token),
            404 => Error::http(format!("Endpoint  ̀{}` not found", stringify!(#fn_name))),
            405 => Error::http(format!("Method {} not allowed (endpoint `̀{}`)", stringify!(#method), stringify!(#fn_name))),
            429 => {
                use chrono::NaiveDateTime;

                let reset = {
                    let reset = response.headers().get("x-ratelimit-reset").unwrap().to_str().unwrap();
                    let mut split_reset = reset.split('.');
                    let secs = split_reset.next().unwrap().parse::<i64>().unwrap();

                    if let Some(m) = split_reset.next() {
                        NaiveDateTime::from_timestamp(secs, m.parse::<u32>().unwrap() * 1_000_000)
                    } else {
                        NaiveDateTime::from_timestamp(secs, 0)
                    }
                };

                Error::rate_limited(stringify!(#fn_name), reset, false)
            },
            502 => Error::http(format!("Gateway unavailable (endpoint `̀{}`)", stringify!(#fn_name))),
            _ => Error::http(format!("Expected status code {}, got {} when requesting {}", #status, code, uri)),
        }
    })
}

/// Parses the list of variables for a Discord API HTTP endpoint.
///   `#[endpoint(get, route = "/gateway/bot", status = 200))]`
///   `#[endpoint(patch, route = "/guilds/{#guild}", body = "modification", status = 200)]`
#[derive(FromMeta)]
struct Args {
    #[darling(default)]
    get: bool,
    #[darling(default)]
    post: bool,
    #[darling(default)]
    put: bool,
    #[darling(default)]
    patch: bool,
    #[darling(default)]
    delete: bool,

    route: String,
    #[darling(default)]
    body: Option<String>,
    #[darling(default)]
    multipart: bool,
    status: u16,
    #[darling(default)]
    empty: bool,
}

impl Args {
    fn method(&self) -> Result<Ident, TokenStream> {
        if self.get {
            Ok(Ident::new("GET", Span::call_site()))
        } else if self.post {
            Ok(Ident::new("POST", Span::call_site()))
        } else if self.put {
            Ok(Ident::new("PUT", Span::call_site()))
        } else if self.patch {
            Ok(Ident::new("PATCH", Span::call_site()))
        } else if self.delete {
            Ok(Ident::new("DELETE", Span::call_site()))
        } else {
            Err(Error::new(Span::call_site(), "Expected method in endpoint macro")
                .to_compile_error()
                .into())
        }
    }

    fn route(&self) -> TokenStream2 {
        let mut quote = quote!(let mut route = String::from("https://discordapp.com/api/v8"););

        for part in self.route.split(&['{', '}'][..]) {
            if part.starts_with('#') {
                let part = Ident::new(part.strip_prefix('#').unwrap(), Span::call_site());

                quote = quote! {
                    #quote
                    let ext: Snowflake = ::automate::encode::ExtractSnowflake::extract_snowflake(&#part)?;
                    ::std::fmt::Write::write_fmt(&mut route, format_args!("{}", ext)).expect("Failed to write api string");
                };
            } else if part.starts_with('+') {
                let part = Ident::new(part.strip_prefix('+').unwrap(), Span::call_site());

                quote = quote! {
                    #quote
                    ::automate::encode::WriteUrl::write_url(#part, &mut route)?;
                };
            } else {
                quote = quote! {
                    #quote
                    ::std::fmt::Write::write_fmt(&mut route, format_args!("{}", #part)).expect("Failed to write api string");
                };
            }
        }

        quote!({#quote route})
    }

    fn body(&self) -> TokenStream2 {
        match self.body.as_ref() {
            Some(body) if self.multipart => {
                let body = Ident::new(body, Span::call_site());
                quote!(::hyper::Body::from(#body))
            },
            Some(body) => {
                let body = Ident::new(body, Span::call_site());
                quote!(::hyper::Body::from(serde_json::to_string(&#body)?))
            },
            None => quote!(::hyper::Body::empty()),
        }
    }

    fn content_type(&self) -> &'static str {
        if self.multipart {
            "multipart/form-data; boundary=--XREJRTlhIaFgKOHZvSG5BOGRqNGxVcWpCWEJhOWQKRllaTG10QWhLNld"
        } else {
            "application/json"
        }
    }
}

/// A Discord HTTP API endpoint.
/// Takes the following parameters:
/// * (get|post|put|patch|delete): defines the HTTP method
/// * route: defines the URL to send the request to
/// * body: The variable in which the body is contained
/// * status: The expected status code
///
/// The route should be a single string. Parameters can
/// be interpolated by being surrounded by curly braces:
/// * Types implementing the `ExtractSnowflake` type,
/// their snowflake will be appended to the URL using
/// [extract_snowflake](automate::encode:ExtractSnowflake::extract_snowflake)
/// (prefix: #).
/// * Types implementing the WriteUrl type, which will
/// be appended to the final string by calling their
/// [write_url](automate::encode::WriteUrl::write_url)
/// method. Useful for types that require a specific
/// formatting or for strings that need to be escaped
/// (prefix: ~).
/// * Expressions that return a type implementing
/// [write_fmt](std::fmt::Write) (no prefix).
///
/// # Example
/// ```ignore
/// use automate::Error;
/// use hyper::Client;
/// use hyper_tls::HttpsConnector;
///
/// pub struct HttpAPI {
///    client: Client<HttpsConnector<HttpConnector>>,
///    token: String,
/// }
///
/// impl HttpAPI {
///    pub fn new(token: &str) -> HttpAPI {
///        let https = HttpsConnector::new();
///
///        let mut bot_token = String::with_capacity(token.len() + 4);
///        bot_token.push_str("Bot ");
///        bot_token.push_str(token);
///
///        HttpAPI {
///            client: Client::builder().build(https),
///            token: bot_token,
///        }
///    }
///
///    #[endpoint(get, route = "/channels/{#channel}/messages/{#message}/reactions/{+emoji}/{query}", status = 200)]
///    pub async fn reactions<S: ExtractSnowflake, U: WriteUrl>(&self, channel: S, message: S, emoji: &U, reactions: ReactionsPosition) -> Result<Vec<User>, Error> {
///        let query = match reactions {
///            ReactionsPosition::Default => String::new(),
///            ReactionsPosition::Limit(limit) => format!("?limit={}", limit),
///            ReactionsPosition::Before(s, limit) => format!("?before={}&limit={}", s, limit),
///            ReactionsPosition::After(s, limit) => format!("?after={}&limit={}", s, limit),
///        };
///    }
/// ```
pub fn endpoint(metadata: TokenStream, item: TokenStream) -> TokenStream {
    let args: AttributeArgs = parse_macro_input!(metadata);
    let args: Args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return e.write_errors().into(); }
    };

    let input: ItemFn = parse_macro_input!(item);
    let visibility = &input.vis;
    let signature = &input.sig;
    let content = &input.block.stmts;

    let mut major_parameter = quote!(None);
    for (name, _) in utils::read_function_arguments(&input.sig) {
        let str_name = name.to_string();

        if &str_name == "guild" || &str_name == "channel" || &str_name == "webhook" {
            major_parameter = quote!(Some(::automate::encode::ExtractSnowflake::extract_snowflake(&#name)?));
        }
    }

    let request = unwrap!(generate_request(&input, args, major_parameter));

    TokenStream::from(quote! {
        #[allow(unused_variables)]
        #visibility #signature {
            #(#content)*
            #request
        }
    })
}