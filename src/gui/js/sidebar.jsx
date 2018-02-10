import React from 'react';
import PropTypes from 'prop-types';
import { AutoSizer, Grid, InfiniteLoader } from 'react-virtualized';
import PhotoThumbnail from './photo-thumbnail';

const COLUMN_WIDTH = 150;

export default class Sidebar extends React.Component {
  constructor(props) {
    super(props);

    this.columnCount = 2;

    this.rowRenderer = this.rowRenderer.bind(this);
    this.rowHeight = this.rowHeight.bind(this);
    this.isRowLoaded = this.isRowLoaded.bind(this);
    this.loadMoreRows = this.loadMoreRows.bind(this);
    this.cellRenderer = this.cellRenderer.bind(this);
    this.onSectionRendered = this.onSectionRendered.bind(this);
  }

  onSectionRendered({
    columnStartIndex,
    columnStopIndex,
    rowStartIndex,
    rowStopIndex
  }) {
    const startIndex = rowStartIndex * this.columnCount + columnStartIndex;
    const stopIndex = rowStopIndex * this.columnCount + columnStopIndex;

    this.onRowsRendered({
      startIndex,
      stopIndex
    });
  }

  cellRenderer({ columnIndex, key, rowIndex, style }) {
    return this.rowRenderer({
      index: rowIndex * this.columnCount + columnIndex,
      key,
      style
    });
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

  scaledPhotoHeight(index) {
    const photo = this.props.photos[index];
    return photo.height / (photo.width / COLUMN_WIDTH);
  }

  rowHeight({ index }) {
    const heights = [];
    for (let i = 0; i < this.columnCount; i += 1) {
      heights.push(this.scaledPhotoHeight(index * this.columnCount + i));
    }
    return Math.max(...heights);
  }

  isRowLoaded({ index }) {
    return this.props.photos[index].loaded;
  }

  loadMoreRows({ startIndex, stopIndex }) {
    return this.props.getAndStoreLocations(startIndex, stopIndex);
  }

  forceUpdate() {
    super.forceUpdate();
    this.grid.recomputeGridSize();
    this.loader.resetLoadMoreRowsCache(true);
  }

  render() {
    return (
      <nav id="photosList">
        <AutoSizer>
          {({ height, width }) => (
            <InfiniteLoader
              ref={loader => {
                this.loader = loader;
              }}
              isRowLoaded={this.isRowLoaded}
              loadMoreRows={this.loadMoreRows}
              rowCount={this.props.photos.length}
            >
              {({ onRowsRendered, registerChild }) => {
                this.columnCount = Math.floor(width / COLUMN_WIDTH);
                this.onRowsRendered = onRowsRendered;
                return (
                  <Grid
                    height={height}
                    width={width}
                    onSectionRendered={this.onSectionRendered}
                    ref={grid => {
                      this.grid = grid;
                      registerChild(grid);
                    }}
                    columnCount={this.columnCount}
                    columnWidth={COLUMN_WIDTH}
                    rowCount={this.props.photos.length / this.columnCount}
                    rowHeight={this.rowHeight}
                    cellRenderer={this.cellRenderer}
                  />
                );
              }}
            </InfiniteLoader>
          )}
        </AutoSizer>
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
  getAndStoreLocations: PropTypes.func.isRequired,
  handlePhotoSelect: PropTypes.func.isRequired
};

Sidebar.defaultProps = {
  currentPhoto: undefined
};
