import * as React from 'react';
import './heading.scss';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { IconProp } from '@fortawesome/fontawesome-svg-core';

export interface LinkItemProps {
  href: string;
  icon: IconProp;
}

const LinkItem = ({ href, icon }: LinkItemProps): JSX.Element => (
  <li className="link-item">
    <a href={href} target="blank">
      <FontAwesomeIcon icon={icon} />
    </a>
  </li>
);

export default LinkItem;
