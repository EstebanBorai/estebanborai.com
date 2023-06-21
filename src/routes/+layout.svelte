<script lang="ts">
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

<div class="bg-light-background-alt dark:bg-dark-background-alt">
  <Header />
  <main class="container safe-zone m-auto py-4">
    <slot />
  </main>
  <Footer />
</div>

<style>
  .container {
    min-height: calc(100vh - 70px);
  }
</style>
