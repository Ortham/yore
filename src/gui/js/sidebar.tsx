import * as React from 'react';
import {
  AutoSizer,
  Grid,
  GridCellProps,
  InfiniteLoader,
  SectionRenderedParams
} from 'react-virtualized';
import { Photo } from './interfaces';
import { PhotoThumbnail } from './photo-thumbnail';

const COLUMN_WIDTH = 150;

export interface SidebarProps {
  photos: Photo[];
  currentPhoto?: Photo;
  handlePhotoSelect: (photo: Photo) => void;
  getAndStoreLocations: (
    startIndex: number,
    stopIndex: number
  ) => Promise<void>;
}

interface IndexRange {
  startIndex: number;
  stopIndex: number;
}

interface RowRendererParameter {
  index: number;
  key: string;
  style: React.CSSProperties;
}

export class Sidebar extends React.Component<SidebarProps, {}> {
  private columnCount: number;
  private grid: Grid;
  private loader: InfiniteLoader;
  private onRowsRendered: (param: IndexRange) => void;

  public constructor(props: SidebarProps) {
    super(props);

    this.columnCount = 2;

    this.rowRenderer = this.rowRenderer.bind(this);
    this.rowHeight = this.rowHeight.bind(this);
    this.isRowLoaded = this.isRowLoaded.bind(this);
    this.loadMoreRows = this.loadMoreRows.bind(this);
    this.cellRenderer = this.cellRenderer.bind(this);
    this.onSectionRendered = this.onSectionRendered.bind(this);
    this.onRowsRendered = undefined;
  }

  public forceUpdate() {
    super.forceUpdate();
    this.grid.recomputeGridSize();
    this.loader.resetLoadMoreRowsCache(true);
  }

  public render() {
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

  private onSectionRendered({
    columnStartIndex,
    columnStopIndex,
    rowStartIndex,
    rowStopIndex
  }: SectionRenderedParams) {
    const startIndex = rowStartIndex * this.columnCount + columnStartIndex;
    const stopIndex = rowStopIndex * this.columnCount + columnStopIndex;

    this.onRowsRendered({
      startIndex,
      stopIndex
    });
  }

  private cellRenderer({ columnIndex, key, rowIndex, style }: GridCellProps) {
    return this.rowRenderer({
      index: rowIndex * this.columnCount + columnIndex,
      key,
      style
    });
  }

  private rowRenderer({ index, key, style }: RowRendererParameter) {
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

  private scaledPhotoHeight(index: number) {
    const photo = this.props.photos[index];
    return photo.height / (photo.width / COLUMN_WIDTH);
  }

  private rowHeight({ index }: { index: number }) {
    const heights = [];
    for (let i = 0; i < this.columnCount; i += 1) {
      heights.push(this.scaledPhotoHeight(index * this.columnCount + i));
    }
    return Math.max(...heights);
  }

  private isRowLoaded({ index }: { index: number }) {
    return this.props.photos[index].loaded;
  }

  private loadMoreRows({ startIndex, stopIndex }: IndexRange) {
    return this.props.getAndStoreLocations(startIndex, stopIndex);
  }
}
