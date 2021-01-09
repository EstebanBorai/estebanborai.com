import React from 'react';
import styled from 'styled-components';

import Picture from './Picture';
import waitGif from '../assets/wait.gif';

const Container = styled.section`
  align-items: center;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-height: calc(100vh - 98px);
`;

const ImageFooter = styled.small`
  border: 1px solid ${(props) => props.theme.textColor};
  box-sizing: border-box;
  color: ${(props) => props.theme.textColor};
  padding: 1rem;
  margin-top: 1rem;
`;

export default function SuspenseFallback(): JSX.Element {
  return (
    <Container>
      <Picture
        src={waitGif}
        alt="Angry cat in front of computer"
        height={480}
        width={384}
      />
      <ImageFooter>Give me a second while I load this for you...</ImageFooter>
    </Container>
  );
}
