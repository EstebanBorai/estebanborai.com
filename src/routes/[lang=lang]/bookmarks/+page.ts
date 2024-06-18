/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const res = await fetch('/db/bookmarks.json');

  if (res.ok) {
    const bookmarks = await res.json();

    return {
      bookmarks,
    };
  }

  return {
    bookmarks: [],
  };
}
