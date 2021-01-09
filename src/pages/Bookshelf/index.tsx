import React, { useEffect, useState } from 'react';
import styled from 'styled-components';

import Subtitle from '@estebanborai.com/components/Subtitle';
import Section from '@estebanborai.com/components/Section';
import { media } from '@estebanborai.com/styles/theme';
import Book from './components/Book';

import type { Website } from '@estebanborai.com/@types/website';

const BookList = styled.ul`
  display: flex;
  flex-wrap: wrap;
  margin: 0 auto;
  padding: 0;
  list-style-type: none;
  margin-top: 2rem;
  min-width: 300px;
`;

const ListItem = styled.li`
  align-items: flex-start;
  display: flex;
  justify-content: flext-start;
  margin-bottom: 1rem;
  width: 50%;

  ${media.md} {
    width: 100%;
  }
`;

export default function Bookshelf(): JSX.Element {
  const [books, setBooks] = useState<Website.Book[] | null>(null);

  useEffect(() => {
    (async () => {
      const mod = await import('./static/books.json');

      setBooks(mod.default as unknown as Website.Book[]);
    })();
  }, []);

  return (
    <Section>
      <header>
        <Subtitle>Bookshelf</Subtitle>
        <p>
          Here you can find some of the books I've read and I would like to
          share with you If we were talking about books and resources to learn
          about software and computer science.
        </p>
      </header>
      <BookList>
        {books && books.length
          ? books.map(
              (book, index): JSX.Element => (
                <ListItem>
                  <Book
                    key={`${book.isbn13}-${index}`}
                    authors={book.authors}
                    coverURL={book.coverURL}
                    title={book.title}
                    isbn13={book.isbn13}
                    bookDepositoryURL={book.bookDepositoryURL}
                  />
                </ListItem>
              ),
            )
          : null}
      </BookList>
    </Section>
  );
}
