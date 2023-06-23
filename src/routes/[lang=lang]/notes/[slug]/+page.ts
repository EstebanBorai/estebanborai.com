export async function load({ params }) {
  const post = await import(`../../../../mdsvex/en/${params.slug}.svx`);
  const content = post.default;

  return {
    content,
  };
}
