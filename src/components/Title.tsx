import styled from 'styled-components';

import { media } from '@estebanborai.com/styles/theme';

const Title = styled.h1`
  font-family: 'Calistoga', cursive;
  font-size: 2rem;

  @media ${media.sm} {
    font-size: 1.75rem;
  }
`;

export default Title;
