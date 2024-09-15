<script lang="ts">
  const title = 'Esteban Borai | Software Developer';
  const description =
    'A Software Developer interested in Systems Programming and Web Development.';
  const avatarUrl = '/images/whoami.jpg';
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <!-- Schema.org markup for Google+ -->
  <meta itemprop="name" content={title} />
  <meta itemprop="description" content={description} />
  <meta itemprop="image" content={avatarUrl} />
  <!-- Open Graph data -->
  <meta property="og:title" content={title} />
  <meta property="og:type" content="article" />
  <meta property="og:url" content="https://estebanborai.com/" />
  <meta property="og:image" content={avatarUrl} />
  <meta property="og:description" content={description} />
  <meta property="og:site_name" content="Esteban Borai" />
  <!-- Twitter Card data -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:site" content="@EstebanBorai" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:creator" content="@EstebanBorai" />
  <meta name="twitter:image:src" content={avatarUrl} />
</svelte:head>

<section class="py-2 px-4">
  <article
    class="gap-4 flex flex-col md:grid md:grid-cols-[140px,auto] mx-auto py-6 md:w-1/2"
  >
    <figure class="col-start-1 col-end-1 rounded-lg overflow-hidden h-32 w-32">
      <img src={avatarUrl} alt="Esteban Borai Profile" />
    </figure>
    <div class="col-start-2 col-end-2">
      <h2 class="inline-block font-semibold pb-2">I'm Esteban,</h2>
      <p class="inline-block pb-2">
        A Software Developer focused in Systems Programming &amp; Web
        Development with {2024 - 2017} years of experience.
      </p>
      <p class="inline-block pb-2">
        Rust &amp; TypeScript are my favorite languages, but I have worked with
        many others, including Python, C# &amp; Go.
      </p>
      <p class="inline-block pb-2">
        Open-Source is where I spend most of my time, where I look for new
        projects to contribute to, as well as experimenting with different
        technologies.
      </p>
    </div>
  </article>
</section>
