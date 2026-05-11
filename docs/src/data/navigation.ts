export const navGroups = [
	{
		title: "Get started",
		items: [
			{ title: "Overview", href: "/docs/" },
			{ title: "Install", href: "/docs/start/install/" },
		],
	},
	{
		title: "Use",
		items: [
			{ title: "Themes", href: "/docs/use/themes/" },
			{ title: "Settings", href: "/docs/use/settings/" },
		],
	},
	{
		title: "Advanced",
		items: [
			{ title: "Prompt segments", href: "/docs/use/prompt-segments/" },
			{ title: "Configuration", href: "/docs/use/configuration/" },
			{ title: "Commands", href: "/docs/reference/commands/" },
			{ title: "Shell setup", href: "/docs/start/shell-setup/" },
			{ title: "Update", href: "/docs/start/update/" },
			{ title: "Uninstall", href: "/docs/start/uninstall/" },
			{ title: "Release artifacts", href: "/docs/reference/release-artifacts/" },
			{ title: "Changelog", href: "/docs/reference/changelog/" },
		],
	},
];

export const quickLinks = navGroups.flatMap((group) => group.items);
