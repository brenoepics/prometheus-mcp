import {VPCarbon} from 'vitepress-carbon'
import {h} from 'vue'
import './override.css'
import HeaderLinks from './components/HeaderLinks.vue'
import DownloadButtons from './components/DownloadButtons.vue'

export default {
    ...VPCarbon,
    Layout: () => {
        return h(VPCarbon.Layout!, null, {
            'nav-bar-content-menu-after': () => h(HeaderLinks),
            'home-hero-image': () => h(DownloadButtons),
        })
    },
}
