import React from 'react';
import styled from 'styled-components';

import Headline from '@estebanborai.com/components/Headline';
import Picture from '@estebanborai.com/components/Picture';
import { media } from '@estebanborai.com/styles/theme';

type Props = {
  title: string;
  authors: string;
  isbn13: string;
  coverURL: string;
  bookDepositoryURL: string;
};

const BookDetails = styled.article`
  display: flex;

  figure {
    display: inline-block;
    width: 120px;
  }

  div {
    ul {
      padding: 0;
      margin: 0;
      list-style-type: none;

      li {
        margin-bottom: 1rem;

        strong {
          display: block;
        }

        span {
          display: block;
        }
      }
    }
  }

  ${media.md} {
    align-items: center;
    flex-direction: column;
    justify-content: center;
    margin-bottom: 2rem;
    width: 100%;

    figure {
      margin: 0 auto;
    }

    h3 {
      padding: 0 1rem;
    }

    h3,
    div ul li {
      text-align: center;
    }

    footer {
      align-items: center;
      display: flex;
      justify-content: center;
      width: 100%;
    }
  }
`;

const BookDepositoryLink = styled.a`
  background-color: ${(props) => props.theme.textColor};
  border-radius: 0.25rem;
  box-sizing: border-box;
  color: ${(props) => props.theme.layoutBackground};
  cursor: pointer;
  display: inline-block;
  font-size: 0.85rem;
  text-decoration: none;
  padding: 0.5rem 0.75rem;
  margin-top: 1rem;
`;

export default function Book({
  title,
  coverURL,
  authors,
  isbn13,
  bookDepositoryURL,
}: Props): JSX.Element {
  return (
    <BookDetails>
      <figure>
        <Picture
          alt={title + ' Book Cover'}
          src={coverURL}
          height={158}
          width={120}
        />
      </figure>
      <div>
        <Headline>{title}</Headline>
        <ul>
          <li>
            <strong>Author(s)</strong>
            <span>{authors}</span>
          </li>
          <li>
            <strong>ISBN-13</strong>
            <span>{isbn13}</span>
          </li>
        </ul>
        <footer>
          <BookDepositoryLink href={bookDepositoryURL} target="_blank">
            Book Depository
          </BookDepositoryLink>
        </footer>
      </div>
    </BookDetails>
  );
}
