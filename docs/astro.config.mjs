// @ts-check
import starlight from "@astrojs/starlight";
import starlightLlmTools from "@wave-rf/starlight-llm-tools";
import { defineConfig } from "astro/config";
import starlightLinksValidator from "starlight-links-validator";
import starlightThemeNext from "starlight-theme-next";

// https://astro.build/config
export default defineConfig({
  site: "https://swelog.rudrapatel.net",
  integrations: [
    starlight({
      plugins: [
        starlightThemeNext(),
        starlightLinksValidator({ sameSitePolicy: "validate" }),
        starlightLlmTools(),
      ],
      title: "swelog",
      description: "A Rust CLI for tracking daily accomplishments in Obsidian.",
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/RudraPatel2003/swelog-cli",
        },
      ],
      sidebar: [
        {
          label: "Getting Started",
          items: [
            { label: "Installation", slug: "getting-started/installation" },
            { label: "Quick Start", slug: "getting-started/quick-start" },
            { label: "Configuration", slug: "getting-started/configuration" },
          ],
        },
        {
          label: "Guides",
          items: [
            { label: "Backfilling a Missed Day", slug: "guides/backfilling" },
            { label: "Undoing a Command", slug: "guides/undo" },
          ],
        },
        {
          label: "Integrations",
          items: [
            { label: "Overview", slug: "integrations" },
            { label: "GitHub", slug: "integrations/github" },
            {
              label: "Google Calendar",
              slug: "integrations/google-calendar",
            },
            { label: "Linear", slug: "integrations/linear" },
            { label: "Sentry", slug: "integrations/sentry" },
          ],
        },
        {
          label: "Summarization",
          items: [
            { label: "Overview", slug: "summarization" },
            { label: "Providers and Models", slug: "summarization/providers" },
          ],
        },
        {
          label: "Reference",
          items: [
            { label: "Commands", slug: "reference/commands" },
            { label: "Authentication", slug: "reference/authentication" },
          ],
        },
        {
          label: "Legal",
          items: [
            { label: "Privacy Policy", slug: "legal/privacy" },
            { label: "Terms of Service", slug: "legal/terms" },
          ],
        },
      ],
    }),
  ],
});
