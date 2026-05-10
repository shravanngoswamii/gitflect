import type { APIRoute } from "astro";
import { generateOgImage } from "../utils/ogImage";

export const GET: APIRoute = async () => {
	const png = await generateOgImage(
		"gitflect",
		"Fast Git context for Unix shells — your branch, status, and counts, right in the prompt.",
	);
	const body = new Uint8Array(png);
	return new Response(body, {
		headers: {
			"Content-Type": "image/png",
			"Cache-Control": "public, max-age=31536000, immutable",
		},
	});
};
