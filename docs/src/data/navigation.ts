export const navGroups = [
	{
		title: 'Start',
		items: [
			{ title: 'Overview', href: '/docs/' },
			{ title: 'Install', href: '/docs/start/install/' },
			{ title: 'Shell setup', href: '/docs/start/shell-setup/' },
			{ title: 'Update', href: '/docs/start/update/' },
		],
	},
	{
		title: 'Use',
		items: [
			{ title: 'Prompt segments', href: '/docs/use/prompt-segments/' },
			{ title: 'Configuration', href: '/docs/use/configuration/' },
			{ title: 'Completions', href: '/docs/use/completions/' },
		],
	},
	{
		title: 'Reference',
		items: [
			{ title: 'Commands', href: '/docs/reference/commands/' },
			{ title: 'Release artifacts', href: '/docs/reference/release-artifacts/' },
		],
	},
];

export const quickLinks = navGroups.flatMap((group) => group.items);
