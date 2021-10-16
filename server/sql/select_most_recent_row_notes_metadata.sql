SELECT
  *
FROM
  notes_metadata nm
WHERE
  nm.updated_at::date = now()::date;
