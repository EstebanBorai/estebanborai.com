import type { Locales } from '$i18n/i18n-types';

export type Slug = string;

export type NoteEntry = {
  slug: Slug;
  lang: keyof Locales;
  title: string;
  date: string;
  categories: string[];
  description: string;
  preview_image_url: string;
  icon: string;
};

export type I18nNoteEntry = {
  slug: Slug;
  langs: keyof Locales[];
  title: string;
  date: string;
  categories: string[];
  description: string;
  preview_image_url: string;
  icon: string;
};

/** @type {import('./$types').RequestHandler} */
export async function GET(e) {
  console.log('params', e);
  const entries = await parseModules();
  const dictionary = collect(entries);
  const sorted = Object.values(dictionary).sort((a, b) =>
    +new Date(a.date) > +new Date(b.date) ? -1 : 1,
  );

  return new Response(JSON.stringify(sorted));
}

async function parseModules(): Promise<NoteEntry[]> {
  const modules = import.meta.glob(
    `../../[lang=lang]/notes/[slug]/mdsvex/**/*.svx`,
  );
  const promises = [];

  for (const [path, resolver] of Object.entries(modules)) {
    const promise = resolver().then((note: { metadata: object }) => {
      const slug = path.match(/mdsvex\/([\w-]+)/i)?.[1] ?? null;
      const lang = path.match(/(en|es|hu)(\.)/i)?.[1] ?? null;

      return {
        slug,
        lang,
        ...note.metadata,
      };
    });

    promises.push(promise);
  }

  return Promise.all(promises);
}

function collect(entries: NoteEntry[]): Record<Slug, I18nNoteEntry> {
  return entries.reduce((acc, entry) => {
    if (acc[entry.slug]) {
      acc[entry.slug].langs.push(entry.lang);
    } else {
      acc[entry.slug] = {
        ...entry,
        langs: [entry.lang],
      };
    }

    return acc;
  }, {});
}
