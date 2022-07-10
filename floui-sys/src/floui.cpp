#include "floui.h"

#define FLOUI_IMPL
#include "../floui/floui.hpp"

void Cfloui_log(const char *s) { floui_log(s); }

#define DEFINE_SHARED_METHODS(x)                                                                   \
    void x##_id(x *self, const char *id) { ((Widget *)self)->id(id); }                             \
    void x##_background(x *self, unsigned int col) { ((Widget *)self)->background(col); }          \
    void x##_size(x *self, int w, int h) { ((Widget *)self)->size(w, h); }                         \
    void *x##_inner(x *self) { return ((Widget *)self)->inner(); }

#define DEFINE_SHARED_GROUP_METHODS(x)                                                             \
    void x##_add(x *self, const CWidget *w) { ((VStack *)self)->add(*(Widget *)w); }               \
    void x##_remove(x *self, const CWidget *w) { ((VStack *)self)->remove(*(Widget *)w); }         \
    void x##_clear(x *self) { ((VStack *)self)->clear(); }                                         \
    void x##_spacing(x *self, int spacing) { ((VStack *)self)->spacing(spacing); }

CFlouiViewController *CFlouiViewController_new(void *arg1, void *arg2, void *arg3) {
    return (CFlouiViewController *)new FlouiViewController(arg1, arg2, arg3);
}

void CFlouiViewController_handle_events(void *view) {
    FlouiViewController::handle_events(view);
}

DEFINE_SHARED_METHODS(CWidget)

CMainView *CMainView_new(const CFlouiViewController *fvc) {
    return (CMainView *)new MainView(*(FlouiViewController *)fvc, {});
}

DEFINE_SHARED_METHODS(CMainView)

DEFINE_SHARED_GROUP_METHODS(CMainView)

CButton *CButton_new(const char *label) { return (CButton *)new Button(label); }

DEFINE_SHARED_METHODS(CButton)

void CButton_action(CButton *self, CFlouiCallback cb, void *data) {
    ((Button *)self)->action([=](Widget &) { (*cb)((CWidget *)self, data); });
}

CText *CText_new(const char *label) { return (CText *)new Text(label); }

DEFINE_SHARED_METHODS(CText)

void CText_text(CText *self, const char *t) { ((Text *)self)->text(t); }
