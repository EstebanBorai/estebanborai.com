INSERT INTO notes_content (content, notes_metadata_id)
    VALUES($1, $2) ON CONFLICT (notes_metadata_id)
    DO
    UPDATE
    SET
      content = EXCLUDED.content,
      updated_at = now()
    RETURNING
      *