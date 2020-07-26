import React from 'react';
import { Link } from "react-router-dom";

type NavLinkProps = {
  children: JSX.Element | JSX.Element[] | string;
  to: string;
};

const NavLink = ({ to, children }: NavLinkProps): JSX.Element => (
  <Link to={to} className="block mt-4 lg:inline-block lg:mt-0 text-teal-200 hover:text-white mr-4">
    {children}
  </Link>
);

export default NavLink;
