use std::str::FromStr;

use log::info;
use rquickjs::function::Args;
use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Ctx, Function, Object};
use tiny_http::{Header, Response, Server, StatusCode};

pub struct Module;

impl ModuleDef for Module {
    #[inline(always)]
    fn declare(declarations: &mut Declarations) -> rquickjs::Result<()> {
        declarations.declare("serve")?;

        Ok(())
    }

    #[inline(always)]
    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> rquickjs::Result<()> {
        exports.export("serve", Function::new(ctx.clone(), Self::serve))?;

        Ok(())
    }
}

impl Module {
    #[inline(always)]
    fn serve(config: Object, serve: Function) {
        let ctx = serve.ctx().clone();

        let address = config
            .get::<&str, String>("address")
            .unwrap_or(String::from("http://127.0.0.1:8080"));
        let log = config.get::<&str, bool>("log").unwrap_or(true);

        let server = Server::http(address.clone())
            .expect(format!("Failed to start server at {}", address).as_str());

        if log {
            info!("HTTP Server started => Listening on {}", address);
        }

        loop {
            if let Ok(req) = server.recv() {
                let result = serve
                    .call_arg::<Object>({
                        let mut func_args = Args::new(serve.ctx().clone(), 1);
                        func_args
                            .push_arg({
                                let obj = Object::new(ctx.clone()).unwrap();

                                obj.set("url", req.url()).unwrap();
                                obj.set("method", req.method().to_string()).unwrap();
                                obj.set(
                                    "headers",
                                    req.headers()
                                        .iter()
                                        .map(|h| h.to_string())
                                        .collect::<Vec<String>>(),
                                )
                                .unwrap();

                                obj
                            })
                            .unwrap();
                        func_args
                    })
                    .expect("Failed to execute serve function");

                if log {
                    info!("HTTP Server // Received Request => {}", {
                        if let Some(addr) = req.remote_addr() {
                            addr.to_string()
                        } else {
                            String::from("{unknown}")
                        }
                    });
                }

                if req.url() == "/close-vanadin-server"
                    && result.get::<&str, bool>("close-on-req").unwrap_or(true)
                {
                    info!("Shutting down due to request to '/close-vanadin-server'");
                    break;
                } else {
                    req.respond(Response::new(
                        StatusCode::from(result.get::<&str, i32>("status").unwrap_or(200)),
                        result
                            .get::<&str, Vec<String>>("headers")
                            .unwrap_or(Vec::new())
                            .iter()
                            .map(|h| Header::from_str(h.as_str()).expect("Failed to parse header"))
                            .collect::<Vec<Header>>(),
                        result
                            .get::<&str, String>("body")
                            .unwrap_or(String::new())
                            .as_bytes(),
                        None,
                        None,
                    ))
                    .expect("Failed to respond to request");
                }
            }
        }
    }
}
