import { Resvg } from "@resvg/resvg-js";
import satori, { type Font } from "satori";

const LOGO_SVG = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 128">
  <path d="M34 72h32c16 0 28-12 28-28" fill="none" stroke="#f5f4f0" stroke-width="11" stroke-linecap="round"/>
  <path d="M66 72c16 0 28 12 28 28" fill="none" stroke="#6ee7b7" stroke-width="11" stroke-linecap="round"/>
  <circle cx="34" cy="72" r="12" fill="#f5f4f0"/>
  <circle cx="94" cy="42" r="12" fill="#eab308"/>
  <circle cx="94" cy="100" r="12" fill="#6ee7b7"/>
</svg>`;

const LOGO_DATA_URI = `data:image/svg+xml;base64,${Buffer.from(LOGO_SVG).toString("base64")}`;

async function fetchGoogleFont(
	family: string,
	weight: number,
	text: string,
): Promise<ArrayBuffer | null> {
	try {
		const css = await fetch(
			`https://fonts.googleapis.com/css2?family=${family}:wght@${weight}&text=${encodeURIComponent(text)}`,
			{
				headers: {
					"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
				},
			},
		).then((r) => r.text());
		const match = css.match(
			/src: url\((.+?)\) format\('(opentype|truetype)'\)/,
		);
		if (!match) return null;
		const res = await fetch(match[1]);
		return res.ok ? res.arrayBuffer() : null;
	} catch {
		return null;
	}
}

let fontsCache: Font[] | null = null;

async function loadFonts(text: string): Promise<Font[]> {
	if (fontsCache) return fontsCache;
	const [regular, bold] = await Promise.all([
		fetchGoogleFont("Inter", 400, text),
		fetchGoogleFont("Inter", 700, text),
	]);
	const fonts: Font[] = [];
	if (regular)
		fonts.push({ name: "Inter", data: regular, weight: 400, style: "normal" });
	if (bold)
		fonts.push({ name: "Inter", data: bold, weight: 700, style: "normal" });
	fontsCache = fonts;
	return fonts;
}

export async function generateOgImage(
	title: string,
	description?: string,
): Promise<Uint8Array> {
	const text = [title, description ?? "", "gitflect", "—"].join(" ");
	const fonts = await loadFonts(text);
	const fontFamily = fonts.length ? "Inter" : "sans-serif";

	const svg = await satori(
		// satori accepts plain VNode objects matching React.ReactNode shape
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		{
			type: "div",
			props: {
				style: {
					width: "100%",
					height: "100%",
					display: "flex",
					flexDirection: "column",
					justifyContent: "center",
					alignItems: "center",
					padding: "72px 80px",
					textAlign: "center",
					background: "#101413",
					fontFamily,
					position: "relative",
					overflow: "hidden",
				},
				children: [
					{
						type: "div",
						props: {
							style: {
								position: "absolute",
								top: "-160px",
								left: "50%",
								marginLeft: "-320px",
								width: "640px",
								height: "640px",
								borderRadius: "50%",
								background:
									"radial-gradient(circle, rgba(110,231,183,0.10) 0%, transparent 65%)",
							},
						},
					},
					{
						type: "div",
						props: {
							style: {
								display: "flex",
								alignItems: "center",
								gap: "20px",
								marginBottom: description ? "36px" : "0",
							},
							children: [
								{
									type: "img",
									props: {
										src: LOGO_DATA_URI,
										width: 90,
										height: 90,
										style: { display: "block" },
									},
								},
								{
									type: "span",
									props: {
										style: {
											fontSize: 72,
											fontWeight: 700,
											color: "#f5f4f0",
											lineHeight: 1,
											letterSpacing: "-0.02em",
										},
										children: "gitflect",
									},
								},
							],
						},
					},
					...(title !== "gitflect"
						? [
								{
									type: "div",
									props: {
										style: {
											fontSize: title.length > 40 ? 44 : 52,
											fontWeight: 700,
											color: "#f5f4f0",
											lineHeight: 1.2,
											maxWidth: "860px",
											marginBottom: description ? "20px" : "0",
										},
										children: title,
									},
								},
							]
						: []),
					...(description
						? [
								{
									type: "div",
									props: {
										style: {
											fontSize: 26,
											color: "rgba(245,244,240,0.55)",
											lineHeight: 1.5,
											maxWidth: "780px",
										},
										children: description,
									},
								},
							]
						: []),
				],
			},
		} as Parameters<typeof satori>[0],
		{
			width: 1200,
			height: 630,
			fonts,
		},
	);

	const resvg = new Resvg(svg, { fitTo: { mode: "width", value: 1200 } });
	return new Uint8Array(resvg.render().asPng());
}
