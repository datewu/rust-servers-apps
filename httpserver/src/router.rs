use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::httprequest::{self, HTTPRequest};
use http::httpresponse::HttpResponse;
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HTTPRequest, stream: &mut impl Write) {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            resp.send_response(stream).unwrap();
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            resp.send_response(stream).unwrap();
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                resp.send_response(stream).unwrap();
            }
        }
    }
}
