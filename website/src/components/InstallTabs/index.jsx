import {useState} from 'react';

import styles from './index.module.css';

const options = [
  {
    id: 'quick',
    label: 'Quick install',
    copy: "curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/asaphaaning/Hyprbaric/master/install.sh | sh",
    code: (
      <>
        <span className={styles.comment}># download the latest package for this Linux distribution</span>{'\n'}
        curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/asaphaaning/Hyprbaric/master/install.sh | sh
      </>
    ),
  },
  {
    id: 'arch',
    label: 'Arch · PKGBUILD',
    copy: 'git clone https://github.com/asaphaaning/Hyprbaric.git\ncd Hyprbaric/packaging/aur\nmakepkg -si',
    code: (
      <>
        <span className={styles.comment}># build the maintained source package</span>{'\n'}
        git clone https://github.com/asaphaaning/Hyprbaric.git{'\n'}
        cd Hyprbaric/packaging/aur{'\n'}
        makepkg -si
      </>
    ),
  },
  {
    id: 'package',
    label: 'Linux packages',
    copy: './packaging/build-linux-packages',
    code: (
      <>
        <span className={styles.comment}># build AppImage, DEB, RPM, and Pacman artifacts</span>{'\n'}
        git clone https://github.com/asaphaaning/Hyprbaric.git{'\n'}
        cd Hyprbaric{'\n'}
        ./packaging/build-linux-packages
      </>
    ),
  },
  {
    id: 'source',
    label: 'From source',
    copy: 'git clone https://github.com/asaphaaning/Hyprbaric.git\ncd Hyprbaric\nflutter pub get\nflutter run -d linux',
    code: (
      <>
        git clone https://github.com/asaphaaning/Hyprbaric.git{'\n'}
        cd Hyprbaric{'\n'}
        flutter pub get{'\n'}
        flutter run -d linux
      </>
    ),
  },
];

export default function InstallTabs() {
  const [selected, setSelected] = useState(options[0]);
  const [copied, setCopied] = useState(false);

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(selected.copy);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1800);
    } catch {
      setCopied(false);
    }
  };

  return (
    <div className={styles.install}>
      <div aria-label="Installation route" className={styles.tabs} role="tablist">
        {options.map((option) => (
          <button
            aria-selected={selected.id === option.id}
            className={selected.id === option.id ? styles.tabActive : styles.tab}
            key={option.id}
            onClick={() => setSelected(option)}
            role="tab"
            type="button">
            {option.label}
          </button>
        ))}
      </div>
      <div className={styles.terminal}>
        <div className={styles.terminalHeader}>
          <span>shell</span>
          <button onClick={copy} type="button">{copied ? 'Copied' : 'Copy'}</button>
        </div>
        <pre>{selected.code}</pre>
      </div>
    </div>
  );
}
