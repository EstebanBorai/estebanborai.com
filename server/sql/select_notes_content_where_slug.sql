SELECT
  nm.id,
  nm.title,
  nm.slug,
  nm.description,
  nm.categories,
  nm. "date",
  nm.sha,
  nm.lang,
  nm.preview_image_url,
  nc. "content"
FROM
  notes_metadata nm
  LEFT JOIN notes_content nc ON nc.notes_metadata_id = nm.id
WHERE
  nm.slug = $1
