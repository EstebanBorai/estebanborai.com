import * as React from 'react';
import './heading.scss';

interface LinkListProps {
  children: JSX.Element | JSX.Element[];
}

const LinkList = ({ children }: LinkListProps): JSX.Element => <ul id="link-list">{children}</ul>;

export default LinkList;
