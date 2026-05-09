# gitflect docs

Astro documentation site for [gitflect](https://shravangoswami.com/gitflect/).

## Development

```sh
npm install
npm run dev        # dev server with hot reload
npm run build      # production build + pagefind index
npm run preview    # preview the built site
```

The dev server runs at `http://localhost:4321/gitflect/` by default.

Content lives in `src/pages/docs/`. The changelog is imported from `../CHANGELOG.md` at the root — edit that file, not a copy inside docs.

## OG image

The shared OG image is generated at build time via `src/pages/og.png.ts` using Satori and `@resvg/resvg-js`. Fonts are fetched from Google Fonts at build time and subsetted to the characters used in the image.
