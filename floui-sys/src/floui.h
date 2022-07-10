#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void Cfloui_log(const char *s);

#define DECLARE_SHARED_METHODS(x)                                                                  \
    void x##_id(x *self, const char *id);                                                          \
    void x##_background(x *self, unsigned int col);                                                \
    void x##_size(x *self, int w, int h);                                                          \
    void *x##_inner(x *self);

#define DECLARE_SHARED_GROUP_METHODS(x)                                                            \
    void x##_add(x *self, const CWidget *w);                                                       \
    void x##_remove(x *self, const CWidget *w);                                                    \
    void x##_clear(x *self);                                                                       \
    void x##_spacing(x *self, int spacing);

typedef struct CFlouiViewController CFlouiViewController;

CFlouiViewController *CFlouiViewController_new(void *, void *, void *);

void CFlouiViewController_handle_events(void *);

typedef struct CWidget CWidget;

DECLARE_SHARED_METHODS(CWidget);

typedef void (*CFlouiCallback)(CWidget *, void *data);

typedef struct CMainView CMainView;

DECLARE_SHARED_METHODS(CMainView);

DECLARE_SHARED_GROUP_METHODS(CMainView);

CMainView *CMainView_new(const CFlouiViewController *fvc);

void CMainView_add(CMainView *self, const CWidget *w);

typedef struct CButton CButton;

CButton *CButton_new(const char *label);

void CButton_action(CButton *self, CFlouiCallback cb, void *data);

typedef struct CText CText;

CText *CText_new(const char *label);

void CText_text(CText *self, const char *t);

#ifdef __cplusplus
}
#endif