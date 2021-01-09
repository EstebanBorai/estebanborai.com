import React from 'react';
import styled from 'styled-components';

import useSettings from '@estebanborai.com/hooks/use-settings';
import Switch from '@estebanborai.com/components/Switch';
import Title from '@estebanborai.com/components/Title';
import { media } from '@estebanborai.com/styles/theme';
import NavLink from './NavLink';

const Header = styled.header`
  width: 100%;

  ${media.md} {
    height: 50px;
  }
`;

const Wrapper = styled.div`
  align-items: center;
  background-color: ${(props) => props.theme.layoutBackground};
  box-sizing: border-box;
  display: flex;
  height: 70px;
  justify-content: space-between;
  margin: 0 auto;
  width: 95%;
`;

const Navigation = styled.div`
  align-items: center;
  display: flex;

  h1 {
    cursor: pointer;
    margin-right: 2rem;
  }

  .home-link {
    color: inherit;
    text-decoration: none;
  }

  ${media.md} {
    h1 {
      font-size: 1.25rem;
    }
  }
`;

const LinkList = styled.ul`
  display: inline-block;
  list-style-type: none;
  margin: 0;
  padding: 0;
`;

export default function Nav(): JSX.Element {
  const settingsContext = useSettings();

  return (
    <Header>
      <Wrapper>
        <Navigation>
          <NavLink exact to="/">
            <Title>Esteban Borai</Title>
          </NavLink>
          <LinkList>
            <NavLink to="/bookshelf" name="Bookshelf" />
          </LinkList>
        </Navigation>
        <div className="header-tools">
          <Switch
            isChecked={settingsContext.theme === 'dark'}
            onChange={settingsContext.toggleTheme}
          />
        </div>
      </Wrapper>
    </Header>
  );
}
