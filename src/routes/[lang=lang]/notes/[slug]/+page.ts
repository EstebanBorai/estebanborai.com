export async function load({ params }) {
  try {
    const post = await import(
      `../../../../mdsvex/${params.slug}/${params.lang}.svx`
    );
    const content = post.default;

    return {
      content,
    };
  } catch (error) {
    const post = await import(`../../../../mdsvex/${params.slug}/en.svx`);
    const content = post.default;

    return {
      content,
    };
  }
}
