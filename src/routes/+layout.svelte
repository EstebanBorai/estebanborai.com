<script lang="ts">
  import '@fontsource/inter';
  import '@fontsource/inter/600.css';
  import { init } from '@sentry/browser';
  import { Integrations } from '@sentry/tracing';
  import { onMount } from 'svelte';

  import { setLocale } from '$i18n/i18n-svelte';

  import Header from '$lib/components/Header.svelte';
  import Footer from '$lib/components/Footer.svelte';

  import type { LayoutData } from './$types';

  import '../app.css';

  export let data: LayoutData;
  // at the very top, set the locale before you access the store and before the actual rendering takes place
  setLocale(data.locale);

  onMount(async () => {
    init({
      dsn: 'https://4f6bf36480d54546a35b1b7740fa7fe5@o1131101.ingest.sentry.io/6179130',
      enabled: true,
      environment: import.meta.env.MODE,
      integrations: [new Integrations.BrowserTracing()],
      tracesSampleRate: 0.3,
    });
  });
</script>

<svelte:head>
  <link rel="icon" type="image/png" href="/favicon-32x32.png" sizes="32x32" />
  <link rel="icon" type="image/png" href="/favicon-16x16.png" sizes="16x16" />
</svelte:head>

<div class="bg-light-background-alt dark:bg-dark-background-alt min-h-screen">
  <Header />
  <main>
    <slot />
  </main>
  <Footer />
</div>
<a hidden rel="me" href="https://hachyderm.io/@estebanborai">Mastodon</a>
