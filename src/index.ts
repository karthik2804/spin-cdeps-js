import { ResponseBuilder } from "@fermyon/spin-sdk";
//@ts-ignore
import { render } from "component:markdown-renderer/markdown-fns"

export async function handler(req: Request, res: ResponseBuilder) {

    let response = render(await req.text());
    res.send(response);
}
