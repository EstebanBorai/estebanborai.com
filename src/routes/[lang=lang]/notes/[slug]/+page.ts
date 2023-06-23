const postUrl = ({ slug, lang }) => `../../../../mdsvex/${slug}/${lang}.svx`;

export async function load({ params }) {
  try {
    const post = await import(
      postUrl({
        slug: params.slug,
        lang: params.lang,
      })
    );
    const content = post.default;

    return {
      content,
    };
  } catch (error) {
    const post = await import(
      /* @vite-ignore */
      postUrl({
        slug: params.slug,
        lang: 'en',
      })
    );
    const content = post.default;

    return {
      content,
    };
  }
}
