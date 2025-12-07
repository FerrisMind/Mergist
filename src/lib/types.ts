export type Tab = 'code' | 'issues';

export interface RepoInfo {
  owner: string;
  repo: string;
  branch?: string | null;
  subdirectory?: string | null;
  original_url: string;
}

export interface Stats {
  files_processed: number;
  files_skipped: number;
  total_size_bytes: number;
  total_lines: number;
  token_count?: number | null;
  total_files?: number;
}

export interface ConversionResult {
  file_path: string;
  stats: Stats;
  repo: RepoInfo;
}

export interface ConvertOptions {
  include_filenames: boolean;
  add_separators: boolean;
  skip_large_files: boolean;
  remove_license_headers: boolean;
  skip_patterns: string[];
}

export interface IssuesExportOptions {
  include_open: boolean;
  include_closed: boolean;
}

export interface IssuesStats {
  total: number;
  open: number;
  closed: number;
  latest_updated?: string | null;
  truncated: boolean;
}

export interface IssuesExportResult {
  file_path: string;
  repo: RepoInfo;
  stats: IssuesStats;
}

export type Status = 'idle' | 'running' | 'success' | 'error';
export type TokenStatus = 'idle' | 'running' | 'success' | 'error';

export interface ConversionState {
  tab: Tab;
  status: Status;
  tokenStatus: TokenStatus;
  progress: { current: number; total: number };
  tokenProgress: number;
  message: string;
  error: string | null;
  result: ConversionResult | null;
  issuesResult: IssuesExportResult | null;
  repoUrl: string;
  skipLargeFiles: boolean;
  removeLicenseHeaders: boolean;
  skipPatterns: string;
  includeOpenIssues: boolean;
  includeClosedIssues: boolean;
}
