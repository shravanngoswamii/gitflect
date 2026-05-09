declare module '@pagefind/default-ui' {
	interface PagefindUIOptions {
		element: string | Element;
		bundlePath?: string;
		showImages?: boolean;
		showSubResults?: boolean;
		excerptLength?: number;
		resetStyles?: boolean;
		processResult?: (result: Record<string, any>) => Record<string, any>;
		[key: string]: unknown;
	}

	export class PagefindUI {
		constructor(options: PagefindUIOptions);
		destroy(): void;
	}
}
