import React from 'react';
import styled from 'styled-components';
import { Link, NavLink as ReactRouterNavLink } from 'react-router-dom';

type Props = {
  to: string;
  name?: string;
  exact?: boolean;
  children?: JSX.Element | JSX.Element[];
};

const LinkWrapper = styled.li`
  cursor: pointer;
  display: inline-block;
  font-family: 'Alata', sans-serif;
  margin-right: 1rem;

  &:last-child {
    margin-right: none;
  }

  a {
    color: ${(props) => props.theme.textColor};
    text-decoration: none;

    &.active {
      text-decoration: underline;
    }

    &:hover {
      color: red;

      transition: 250ms background-color ease-in;
    }
  }
`;

export default function NavLink({
  exact,
  children,
  to,
  name,
}: Props): JSX.Element {
  if (children) {
    return (
      <LinkWrapper>
        <Link to={to}>{children}</Link>
      </LinkWrapper>
    );
  }

  return (
    <LinkWrapper>
      <ReactRouterNavLink exact={exact} to={to}>
        {name}
      </ReactRouterNavLink>
    </LinkWrapper>
  );
}
