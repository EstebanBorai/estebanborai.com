export const ssr = false;

/** @type {import('./$types').PageLoad} */
export async function load({ fetch }) {
  const res = await fetch('/db/notes_index.json');

  if (res.ok) {
    const notesIndex = await res.json();

    return {
      notesIndex: notesIndex.slice(0, 5),
    };
  }

  return {
    notesIndex: [],
  };
}
