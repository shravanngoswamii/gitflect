import satori, { type Font } from 'satori';
import { Resvg } from '@resvg/resvg-js';

async function fetchGoogleFont(family: string, weight: number, text: string): Promise<ArrayBuffer | null> {
	try {
		const css = await fetch(
			`https://fonts.googleapis.com/css2?family=${family}:wght@${weight}&text=${encodeURIComponent(text)}`,
			{ headers: { 'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36' } },
		).then((r) => r.text());
		const match = css.match(/src: url\((.+?)\) format\('(opentype|truetype)'\)/);
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
		fetchGoogleFont('Inter', 400, text),
		fetchGoogleFont('Inter', 700, text),
	]);
	const fonts: Font[] = [];
	if (regular) fonts.push({ name: 'Inter', data: regular, weight: 400, style: 'normal' });
	if (bold) fonts.push({ name: 'Inter', data: bold, weight: 700, style: 'normal' });
	fontsCache = fonts;
	return fonts;
}

export async function generateOgImage(title: string, description?: string): Promise<Uint8Array> {
	const text = [title, description ?? '', 'gitflect'].join(' ');
	const fonts = await loadFonts(text);
	const fontFamily = fonts.length ? 'Inter' : 'sans-serif';

	const svg = await satori(
		// satori accepts plain VNode objects matching React.ReactNode shape
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		({
			type: 'div',
			props: {
				style: {
					width: '100%',
					height: '100%',
					display: 'flex',
					flexDirection: 'column',
					justifyContent: 'center',
					alignItems: 'flex-start',
					padding: '72px 80px',
					background: '#101413',
					fontFamily,
					position: 'relative',
					overflow: 'hidden',
				},
				children: [
					{
						type: 'div',
						props: {
							style: {
								position: 'absolute',
								top: '-120px',
								right: '-80px',
								width: '480px',
								height: '480px',
								borderRadius: '50%',
								background: 'radial-gradient(circle, rgba(110,231,183,0.12) 0%, transparent 70%)',
							},
						},
					},
					{
						type: 'div',
						props: {
							style: {
								display: 'flex',
								alignItems: 'center',
								gap: '10px',
								marginBottom: '28px',
							},
							children: [
								{
									type: 'div',
									props: {
										style: {
											width: '8px',
											height: '8px',
											borderRadius: '50%',
											background: '#6ee7b7',
										},
									},
								},
								{
									type: 'span',
									props: {
										style: {
											fontSize: 14,
											fontWeight: 700,
											color: 'rgba(110,231,183,0.7)',
											letterSpacing: '0.12em',
											textTransform: 'uppercase',
										},
										children: 'gitflect',
									},
								},
							],
						},
					},
					{
						type: 'div',
						props: {
							style: {
								fontSize: title.length > 40 ? 48 : 58,
								fontWeight: 700,
								color: '#f5f4f0',
								lineHeight: 1.15,
								maxWidth: '860px',
								marginBottom: description ? '24px' : '0',
							},
							children: title,
						},
					},
					...(description
						? [
								{
									type: 'div',
									props: {
										style: {
											fontSize: 24,
											color: 'rgba(245,244,240,0.55)',
											lineHeight: 1.5,
											maxWidth: '760px',
										},
										children: description,
									},
								},
							]
						: []),
				],
			},
		} as Parameters<typeof satori>[0]),
		{
			width: 1200,
			height: 630,
			fonts,
		},
	);

	const resvg = new Resvg(svg, { fitTo: { mode: 'width', value: 1200 } });
	return new Uint8Array(resvg.render().asPng());
}
