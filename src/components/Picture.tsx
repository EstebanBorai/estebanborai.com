import React from 'react';
import styled from 'styled-components';

import useSettings from '@estebanborai.com/hooks/use-settings';

import type { CSSProperties } from 'react';

type Props = {
  alt: string;
  src: string;
  height: number;
  style?: CSSProperties;
  width: number;
};

type ImageInternalProps = {
  isDark: boolean;
};

const Image = styled.img<Props & ImageInternalProps>`
  ${(props) => (props.isDark ? 'filter: grayscale(100%);' : '')}
  transition: 250ms filter ease;

  &:hover {
    ${(props) => (props.isDark ? 'filter: grayscale(0%);' : '')}
    transition: 250ms filter ease;
  }
`;

export default function Picture({
  src,
  alt,
  height,
  style,
  width,
}: Props): JSX.Element {
  const settings = useSettings();

  return (
    <Image
      alt={alt}
      src={src}
      height={height}
      width={width}
      style={style}
      isDark={settings.theme === 'dark'}
    />
  );
}
