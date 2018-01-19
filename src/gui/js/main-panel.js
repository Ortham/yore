import React from 'react';
import MapArea from './map-area';
import {locationDescription, hasSuggestion} from './photo';

export default function MainPanel(props) {
    if (props.photo) {
        return (
            <main>
                <section>
                    <img src={props.photo.src} />
                    <MapArea photo={props.photo} />
                </section>
                <footer>
                    {locationDescription(props.photo)}
                    <br />
                    <button
                        disabled={!hasSuggestion(props.photo)}
                        onClick={props.handleSuggestionApply} >
                        Apply
                    </button>
                    <button
                        disabled={!hasSuggestion(props.photo)}
                        onClick={props.handleSuggestionDiscard} >
                        Discard
                    </button>
                </footer>
            </main>
        );
    } else {
        return (
            <main>
                <section>No photo selected.</section>
                <footer>
                    <br />
                    <button disabled>Apply</button>
                    <button disabled>Discard</button>
                </footer>
            </main>
        );
    }
}
