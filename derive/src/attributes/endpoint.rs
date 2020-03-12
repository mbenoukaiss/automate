use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, ItemFn, Error};
use quote::quote;
use crate::utils;
use syn::spanned::Spanned;

/// Default user agent for automate bots
const USER_AGENT: &str = concat!("DiscordBot (https://github.com/mbenoukaiss/automate, ", env!("CARGO_PKG_VERSION"), ")");

/// Generates the code that should build
/// the route URL based on the parameters.
fn generate_route(route: String, span: Span) -> proc_macro2::TokenStream {
    let mut quote = quote!(let mut route = String::from("https://discordapp.com/api/v6"););

    for part in route.trim_matches('"').split(&['{', '}'][..]) {
        if part.starts_with('#') {
            let part = Ident::new(&part[1..].to_owned(), span);

            quote = quote! {
                #quote
                let ext: Snowflake = ::automate::encode::ExtractSnowflake::extract_snowflake(&#part)?;
                ::std::fmt::Write::write_fmt(&mut route, format_args!("{}", ext)).expect("Failed to write api string");
            };
        } else if part.starts_with('+') {
            let part = Ident::new(&part[1..], span);

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

/// Generate the code that should send the request
/// to Discord's servers.
fn generate_request(uri: proc_macro2::TokenStream, method: Ident, body: proc_macro2::TokenStream, status: Vec<u16>, empty: bool) -> proc_macro2::TokenStream {
    // hyper does not set content-length to 0 when the body is
    // empty and method is POST, PUT or PATCH, but discord
    // requires a content-length
    let zero_content_length = match method.to_string().as_str() {
        "POST" | "PUT" | "PATCH" => Some(quote! {
            match ::hyper::body::HttpBody::size_hint(&#body).exact() {
                Some(0) | None => request = request.header("Content-Length", 0),
                _ => ()
            };
        }),
        _ => None
    };

    let return_value = if empty {
        quote!(Ok(()))
    } else {
        quote! {
            let body = ::hyper::body::aggregate(response).await?;
            Ok(::serde_json::from_reader(::bytes::buf::ext::BufExt::reader(body))?)
        }
    };

    quote! {
        let uri = #uri;

        let mut request = ::hyper::Request::builder()
            .uri(uri.clone())
            .method(::hyper::Method::#method)
            .header("Content-Type", "application/json")
            .header("Authorization", &self.token)
            .header("User-Agent", #USER_AGENT);

        #zero_content_length

        let response = self.client.request(request.body(#body).unwrap()).await?;

        if self.bucket(response.headers()).is_err() {
            trace!("Failed to retrieve bucket from {:#?} ({})", response.headers(), uri);
        }

        #(
            let code = response.status().as_u16();

            if #status != code {
                return Error::err(format!("Expected status code {}, got {} when requesting {}", #status, code, uri));
            }
        )*

        #return_value
    }
}

/// A Discord HTTP API endpoint.
/// Takes the following parameters :
/// * (get|post|put|patch|delete): defines the HTTP method
/// * route: defines the URL to send the request to
/// * body: The variable in which the body is contained
/// * status: The expected status code
///
/// The route should be a single string. Parameters can
/// be interpolated by being surrounded by curly braces :
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
    let arguments = utils::parse_arguments_list(metadata);

    let input = parse_macro_input!(item as ItemFn);
    let span = input.span();
    let visibility = &input.vis;
    let signature = &input.sig;
    let content = &input.block;

    let method = if arguments.contains_key("get") {
        Ident::new("GET", span)
    } else if arguments.contains_key("post") {
        Ident::new("POST", span)
    } else if arguments.contains_key("put") {
        Ident::new("PUT", span)
    } else if arguments.contains_key("patch") {
        Ident::new("PATCH", span)
    } else if arguments.contains_key("delete") {
        Ident::new("DELETE", span)
    } else {
        return Error::new(span, "Expected method in endpoint macro")
            .to_compile_error()
            .into();
    };

    let route = generate_route(utils::extract_string(&arguments, "route"), span);

    let body = match arguments.get("body") {
        Some(body) => {
            let token = body.first().unwrap();
            let body = Ident::new(token.to_string().trim_matches('"'), token.span().into());

            quote!(::hyper::Body::from(::automate::encode::AsJson::as_json(&#body)))
        }
        None => quote!(::hyper::Body::empty()),
    };

    let status = match arguments.get("status") {
        Some(status) => {
            let status = {
                if let Some(TokenTree::Literal(status)) = status.first() {
                    if let Ok(status) = status.to_string().parse::<u16>() {
                        status
                    } else {
                        return Error::new(span, "Failed to parse status code as u16")
                            .to_compile_error()
                            .into();
                    }
                } else {
                    return Error::new(span, "Expected u16 for status code")
                        .to_compile_error()
                        .into();
                }
            };

            vec![status]
        }
        None => vec![],
    };

    let request = generate_request(route, method, body, status, arguments.get("empty").is_some());

    TokenStream::from(quote! {
        #[allow(unused_variables)]
        #visibility #signature {
            #content
            #request
        }
    })
}