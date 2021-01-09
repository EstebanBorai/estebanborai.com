import React from 'react';
import styled from 'styled-components';

import SettingsContext from '../context/settings';

type Props = {
  children: JSX.Element | JSX.Element[];
};

type State = {
  error?: Error;
  hasError: boolean;
  showError: boolean;
};

const Section = styled.div`
  align-items: center;
  background-color: #000000;
  box-sizing: border-box;
  color: #00aa00;
  display: flex;
  font-family: 'Inconsolata', monospace;
  flex-direction: column;
  height: 100vh;
  justify-content: center;
  transition: all 250ms ease-out;
  padding: 0 5%;
  width: 100%;
`;

const Button = styled.a`
  background-color: #000000;
  border: 1px solid #aaaaaa;
  border-radius: 0.5rem;
  box-sizing: border-box;
  color: #00aa00;
  text-decoration: none;
  padding: 1rem 2rem;
  margin-bottom: 1rem;
`;

export default class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      error: undefined,
      hasError: false,
      showError: false,
    };
  }

  static contextType = SettingsContext;

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, showError: false };
  }

  componentDidCatch(error: Error, errorInfo: unknown): void {
    console.error(error, errorInfo);
  }

  render(): JSX.Element | JSX.Element[] {
    if (this.state.hasError) {
      return (
        <Section>
          <h1>Something went wrong.</h1>
          <p>
            An unhandled error ocurred in my website. Please, help me maintain
            my website compatibility and stability by opening an issue!
          </p>
          <Button
            href="https://github.com/EstebanBorai/estebanborai.com/issues/new"
            target="_blank"
          >
            Open an issue on GitHub
          </Button>
          <Button href="./">reset --hard</Button>
        </Section>
      );
    }

    return this.props.children;
  }
}
