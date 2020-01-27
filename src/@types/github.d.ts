/**
 * UserResponse represents the data received from GitHub API
 * when making a GET Request to "/users/:username"
 */
export interface UserResponse {
  login: string;
  id: number;
  nodeId: string;
  avatarUrl: string;
  gravatarId: string;
  url: string;
  htmlUrl: string;
  followersUrl: string;
  followingUrl: string;
  gistsUrl: string;
  starredUrl: string;
  subscriptionsUrl: string;
  reposUrl: string;
  eventsUrl: string;
  receivedEventsUrl: string;
  type: string;
  siteAdmin: boolean;
  name: string;
  company: string;
  blog: string;
  location: string;
  email: string;
  hireable: boolean;
  bio: string;
  publicRepos: number;
  publicGists: number;
  followers: number;
  following: number;
  createdAt: Date;
  updatedAt: Date;
}
