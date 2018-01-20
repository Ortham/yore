import React from 'react';
import PropTypes from 'prop-types';
import { AutoSizer, InfiniteLoader, List } from 'react-virtualized';
import PhotoThumbnail from './photo-thumbnail';

export default class Sidebar extends React.Component {
  constructor(props) {
    super(props);

    this.rowRenderer = this.rowRenderer.bind(this);
    this.rowHeight = this.rowHeight.bind(this);
    this.isRowLoaded = this.isRowLoaded.bind(this);
    this.loadMoreRows = this.loadMoreRows.bind(this);
    this.handleFilterToggle = this.handleFilterToggle.bind(this);
  }

  rowRenderer({ index, key, style }) {
    const photo = this.props.photos[index];
    return (
      <PhotoThumbnail
        key={key}
        style={style}
        isSelected={photo === this.props.currentPhoto}
        photo={photo}
        handleSelect={() => this.props.handlePhotoSelect(photo)}
      />
    );
  }

  rowHeight({ index }) {
    const photo = this.props.photos[index];
    return photo.height / (photo.width / 272);
  }

  isRowLoaded({ index }) {
    return this.props.photos[index].loaded;
  }

  loadMoreRows({ startIndex, stopIndex }) {
    return this.props.getAndStoreLocations(startIndex, stopIndex);
  }

  handleFilterToggle(event) {
    return this.props.handleFilterToggle(event).then(() => {
      this.list.recomputeRowHeights();
    });
  }

  forceUpdate() {
    super.forceUpdate();
    this.list.forceUpdateGrid();
  }

  render() {
    return (
      <nav id="sidebar">
        <div id="photosList">
          <AutoSizer>
            {({ height, width }) => (
              <InfiniteLoader
                isRowLoaded={this.isRowLoaded}
                loadMoreRows={this.loadMoreRows}
                rowCount={this.props.photos.length}
              >
                {({ onRowsRendered, registerChild }) => (
                  <List
                    height={height}
                    onRowsRendered={onRowsRendered}
                    ref={list => {
                      this.list = list;
                      registerChild(list);
                    }}
                    rowCount={this.props.photos.length}
                    rowHeight={this.rowHeight}
                    rowRenderer={this.rowRenderer}
                    width={width}
                  />
                )}
              </InfiniteLoader>
            )}
          </AutoSizer>
        </div>
        <footer>
          <label htmlFor="suggestionsCheckbox">
            <input
              type="checkbox"
              id="suggestionsCheckbox"
              checked={this.props.filterPhotos}
              onChange={this.handleFilterToggle}
            />
            Show only photos with suggestions
          </label>
        </footer>
      </nav>
    );
  }
}

const photoType = PropTypes.shape({
  height: PropTypes.number,
  width: PropTypes.number,
  loaded: PropTypes.bool,
  path: PropTypes.string,
  src: PropTypes.string
});

Sidebar.propTypes = {
  photos: PropTypes.arrayOf(photoType).isRequired,
  currentPhoto: photoType,
  filterPhotos: PropTypes.bool.isRequired,
  getAndStoreLocations: PropTypes.func.isRequired,
  handleFilterToggle: PropTypes.func.isRequired,
  handlePhotoSelect: PropTypes.func.isRequired
};

Sidebar.defaultProps = {
  currentPhoto: undefined
};
