export async function load({ fetch }) {
  const res = await fetch('/api/notes');

  if (res.ok) {
    const { notes } = await res.json();

    return {
      props: {
        notes,
      },
    };
  }

  return {
    status: res.status,
  };
}
