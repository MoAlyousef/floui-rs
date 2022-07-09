#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CFlouiViewController CFlouiViewController;

CFlouiViewController *CFlouiViewController_new(void *, void *, void *);

typedef struct CWidget CWidget;

typedef void (*CFlouiCallback)(CWidget *, void *data);

typedef struct CMainView CMainView;

CMainView *CMainView_new(const CFlouiViewController *fvc);

void CMainView_add(CMainView *self, const CWidget *w);

typedef struct CButton CButton;

CButton *CButton_new(const char *label);

void CButton_action(CButton *self, CFlouiCallback cb, void *data);

typedef struct CText CText;

CText *CText_new(const char *label);

void CText_text(CText *self, const char *t);

void Cfloui_log(const char *s);

#ifdef __cplusplus
}
#endif