async function listNotes() {
  const modules = import.meta.glob('../../notes/*.svx');
  const promises = [];

  for (const [path, resolver] of Object.entries(modules)) {
    const promise = resolver().then((note: { metadata: object }) => {
      const slug = path.match(/([\w-]+)(\.)/i)?.[1] ?? null;

      return {
        slug,
        ...note.metadata,
      };
    });

    promises.push(promise);
  }

  const notes = await Promise.all(promises);
  const sorted = notes.sort((a, b) =>
    +new Date(a.date) > +new Date(b.date) ? -1 : 1,
  );

  return sorted;
}

export async function GET(): Promise<{ body: Record<string, unknown> }> {
  const notes = await listNotes();

  return new Response(
    JSON.stringify({
      notes,
    }),
  );
}
