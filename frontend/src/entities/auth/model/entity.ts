export class AuthenticatedUser {
  constructor(
    public readonly uid: string,
    public username: string,
  ) {}
}
