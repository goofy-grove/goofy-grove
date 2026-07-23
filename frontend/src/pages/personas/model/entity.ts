export class Persona {
  constructor(
    public readonly uid: string,
    public name: string,
    public description: string,
    public creatorUid: string,
    public avatarUid: string | null = null,
  ) {}
}
