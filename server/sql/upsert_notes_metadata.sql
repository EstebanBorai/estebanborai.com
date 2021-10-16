INSERT INTO notes_metadata (title, slug, description, categories, date, lang, sha)
    VALUES($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (slug)
    DO
    UPDATE
    SET
      sha = EXCLUDED.sha,
      updated_at = now()
    RETURNING
      *