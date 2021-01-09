import React from 'react';
import styled from 'styled-components';

import { media } from '@estebanborai.com/styles/theme';
import Picture from '@estebanborai.com/components/Picture';
import GitHubLogo from '@estebanborai.com/components/icon/GitHub';
import TwitterLogo from '@estebanborai.com/components/icon/Twitter';
import LinkedInLogo from '@estebanborai.com/components/icon/LinkedIn';

const Section = styled.section`
  align-items: center;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - 70px);
  justify-content: center;
  position: relative;

  ${media.xs} {
    margin-top: 70px;
  }

  ${media.md} {
    margin-top: 70px;
  }
`;

const Illustration = styled.div`
  display: inline-block;
  margin: 0 3rem 0 auto;

  figure {
    height: 320px;
    margin: 0 auto;
    width: 320px;

    img {
      height: 320px;
      width: 320px;
    }
  }

  ${media.md} {
    align-items: center;
    display: flex;
    margin: 0 auto;
    margin-bottom: 2rem;
    height: 240px;
    justify-content: center;
    width: 240px;

    figure {
      height: 240px;
      width: 240px;

      img {
        height: 240px;
        width: 240px;
      }
    }
  }
`;

const Introduction = styled.div`
  display: inline-block;
  margin: 0 auto 0 0;
  width: 45%;

  ${media.xs} {
    margin-top: 70px;
  }

  ${media.md} {
    margin-top:: 70px;
    width: 100%;
  }
`;

const Greeting = styled.p`
  font-family: ${(props) => props.theme.fonts.title};
  font-size: 3rem;
  text-align: left;
  margin: 0;
  margin-bottom: 2rem;

  ${media.xs} {
    margin-top: 30px;
  }

  ${media.md} {
    display: block;
    font-size: 2rem;
    text-align: center;
    width: 100%;
  }
`;

const Bio = styled.p`
  color: ${(props) => props.theme.fonts.body};
  font-family: ${(props) => props.theme.fonts.body};
  font-size: 1.1rem;
  text-align: left;
  margin: 0;

  ${media.md} {
    font-size: 1rem;
    text-align: center;
    margin: 0 auto;
    margin-bottom: 2rem;
    width: 90%;
  }
`;

const Wrapper = styled.div`
  align-items: center;
  box-sizing: border-box;
  display: flex;
  justify-content: center;

  ${media.xs} {
    margin-top: 70px;
  }

  ${media.md} {
    align-items: center;
    flex-direction: column;
    height: calc(100vh - 50px);
    justify-content: center;
  }
`;

const SocialLinks = styled.ul`
  align-items: center;
  display: flex;
  height: 60px;
  justify-content: center;
  list-style: none;
  margin: 0;
  padding: 0;
  width: 100%;

  li {
    align-items: center;
    display: flex;
    fill: ${(props) => props.theme.textColor};
    justify-content: center;
    margin-right: 2rem;
    height: 32px;
    width: 32px;

    &:last-child {
      margin-right: 0;
    }
  }
`;

export default function Home(): JSX.Element {
  return (
    <Section>
      <Wrapper>
        <Illustration>
          <figure>
            <Picture
              src="https://avatars.githubusercontent.com/u/34756077?v=4"
              alt="Esteban Borai Avatar"
              height={250}
              width={250}
              style={{
                display: 'inline-block',
                borderRadius: '9999px',
              }}
            />
          </figure>
        </Illustration>
        <Introduction>
          <Greeting>Hi, I'm Esteban!</Greeting>
          <Bio>
            I'm a Software Engineer interested in Systems Programming and Web
            Development.
            <br />
            <br />
            I'm curious about low-level programming, algorithms, data structures
            and computer science in general.
            <br />
            <br />I love sharing my knowledge with others, building in teams,
            contributing to open source, checking out different technologies,
            play videogames, read books about software, watch movies and
            traveling.
          </Bio>
        </Introduction>
      </Wrapper>
      <SocialLinks>
        <li>
          <a
            className="github-logo"
            href="https://github.com/EstebanBorai"
            target="_blank"
          >
            <GitHubLogo height={32} width={32} />
          </a>
        </li>
        <li>
          <a
            className="twitter-logo"
            href="https://twitter.com/EstebanBorai"
            target="_blank"
          >
            <TwitterLogo height={32} width={32} />
          </a>
        </li>
        <li>
          <a
            className="linkedin-logo"
            href="https://www.linkedin.com/in/esteban-b-241ba0135/"
            target="_blank"
          >
            <LinkedInLogo height={32} width={32} />
          </a>
        </li>
      </SocialLinks>
    </Section>
  );
}
