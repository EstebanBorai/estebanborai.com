import styled from 'styled-components';

import { media } from '@estebanborai.com/styles/theme';

const Subtitle = styled.h2`
  font-family: 'Alata', sans-serif;
  font-size: 1.6rem;
  margin: 0;

  @media ${media.sm} {
    font-size: 1.45rem;
  }
`;

export default Subtitle;
