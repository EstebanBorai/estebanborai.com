import styled from 'styled-components';

import { media } from '@estebanborai.com/styles/theme';

const Headline = styled.h3`
  font-family: 'Alata', serif;
  font-size: 1.2rem;

  @media ${media.sm} {
    font-size: 1.35rem;
  }
`;

export default Headline;
