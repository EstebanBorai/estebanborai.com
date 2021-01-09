import React from 'react';
import styled from 'styled-components';

import { media } from '@estebanborai.com/styles/theme';

type Props = {
  isChecked: boolean;
  onChange(value: boolean): void;
};

const Label = styled.label``;

const Checkbox = styled.input`
  display: none;

  &:checked + div {
    background-color: #ffffff;
    border-color: #000000;
  }

  &:checked + div:before {
    background-color: #000000;
    left: 23px;
  }
`;

const SwitchButton = styled.div`
  background-color: #000000;
  border: 1px solid #ffffff;
  border-radius: 15px;
  cursor: pointer;
  height: 30px;
  position: relative;
  width: 50px;

  &:before {
    background-color: #ffffff;
    content: '';
    position: absolute;
    height: 25px;
    width: 25px;
    left: 2px;
    top: 50%;
    transition: all 250ms ease-out;
    cursor: pointer;
    border-radius: 50%;
    transform: translateY(-50%);
  }

  ${media.md} {
    height: 25px;
    width: 45px;

    &:before {
      height: 20px;
      width: 20px;
    }
  }
`;

export default function Switch({ isChecked, onChange }: Props): JSX.Element {
  const handleChange = (): void => {
    onChange(!isChecked);
  };

  return (
    <Label>
      <Checkbox type="checkbox" checked={isChecked} onChange={handleChange} />
      <SwitchButton />
    </Label>
  );
}
