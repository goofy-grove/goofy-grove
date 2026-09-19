import './styles.scss';

export const GroveScene = () => (
  <div className="grove-scene" aria-hidden="true">
    <span className="grove-scene__sun" />

    <span className="grove-scene__tree grove-scene__tree--left">
      <i />
    </span>

    <span className="grove-scene__tree grove-scene__tree--middle">
      <i />
    </span>

    <span className="grove-scene__tree grove-scene__tree--right">
      <i />
    </span>

    <span className="grove-scene__ground" />
  </div>
);
